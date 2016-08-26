/*
 * RustyPlug - a rust module with a fluid interface for building requests to sockets
 *
 * Copyright (C) 2016 Steve G. Bjorg
 *
 * For community documentation and downloads visit mindtouch.com;
 * please review the licensing section.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq)]
pub enum UriCredentials {
    None,
    Username(String),
    UsernamePassword(String, String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum UriParserError {
    InternalError,
    InvalidScheme,
    InvalidHostname,
    InvalidIPv6,
    InvalidPortNumber
}

pub fn parse_scheme(parser: &mut Peekable<Chars>) -> Result<String, UriParserError> {

    // scheme must begin with alphabetic character
    match parser.next() {
        Some(first) if
            ((first >= 'a') && (first <= 'z'))
            || ((first >= 'A') && (first <= 'Z'))
        => {
            let mut buffer = String::new();
            buffer.push(first);
            loop {
                match parser.peek() {
                    Some(&c) if
                        ((c >= 'a') && (c <= 'z'))
                        || ((c >= 'A') && (c <= 'Z'))
                        || ((c >= '0') && (c <= '9'))
                    => {
                        parser.next();

                        // valid character, keep parsing
                        buffer.push(c);
                    },
                    Some(&':') | None => {
                        return Ok(buffer);
                    }
                    _ => return Err(UriParserError::InvalidScheme)
                }
            }
        },
        _ => return Err(UriParserError::InvalidScheme)
    };
}

pub fn parse_authority(mut parser: &mut Peekable<Chars>) -> Result<(UriCredentials,String,Option<u16>), UriParserError> {
    fn parse_hostname_or_userinfo(mut parser: &mut Peekable<Chars>) -> Result<(UriCredentials,String,Option<u16>), UriParserError> {
        let mut decode = false;
        let mut buffer = String::new();

        // parse hostname -OR- user-info
        loop {
            match parser.peek() {
                Some(&c) if (c == '%') || (c <= '+') => {
                    parser.next();
                    decode = true;

                    // potentially valid character, keep parsing
                    buffer.push(c);
                },
                Some(&c) if
                    ((c >= 'a') && (c <= 'z'))
                    || ((c >= 'A') && (c <= 'Z'))
                    || ((c >= '0') && (c <= '9'))
                    || ((c >= '$') && (c <= '.'))   // one of: $%&'()*+,-.
                    || (c == '!')
                    || (c == ';')
                    || (c == '=')
                    || (c == '_')
                    || (c == '~')
                    || c.is_numeric()
                    || c.is_alphabetic()
                => {
                    parser.next();

                    // valid character, keep parsing
                    buffer.push(c);
                },
                Some(&':') => {
                    parser.next();

                    // part before ':' is either a username or hostname
                    let decode_hostname_or_username = decode;
                    decode = false;
                    let hostname_or_username = buffer;
                    buffer = String::new();

                    // parse either password -OR- port number
                    loop {
                        match parser.peek() {
                            Some(&c) if (c == '%') || (c <= '+') => {
                                parser.next();
                                decode = true;

                                // potentially valid character, keep parsing
                                buffer.push(c);
                            },
                            Some(&c) if
                                ((c >= 'a') && (c <= 'z'))
                                || ((c >= 'A') && (c <= 'Z'))
                                || ((c >= '0') && (c <= '9'))
                                || ((c >= '$') && (c <= '.'))   // one of: $%&'()*+,-.
                                || (c == '!')
                                || (c == ';')
                                || (c == '=')
                                || (c == '_')
                                || (c == '~')
                                || c.is_numeric()
                                || c.is_alphabetic()
                            => {
                                parser.next();

                                // valid character, keep parsing
                                buffer.push(c);
                            },
                            Some(&'@') => {
                                parser.next();

                                // part before ':' was username
                                let username = if decode_hostname_or_username {
                                    uri_decode(&hostname_or_username)
                                } else {
                                    hostname_or_username
                                };

                                // part after ':' is password
                                let password = if decode {
                                    uri_decode(&buffer)
                                } else {
                                    buffer
                                };
                                let credentials = UriCredentials::UsernamePassword(username, password);

                                // continue with parsing the hostname or IPv6 address
                                if Some(&'[') == parser.peek() {
                                    return parse_ipv6(parser, credentials);
                                }
                                return parse_hostname(parser, credentials);
                            },
                            Some(&'/') | Some(&'\\') | Some(&'?') | Some(&'#') | None => {

                                // part before ':' was hostname
                                if decode_hostname_or_username {

                                    // hostname cannot contain encoded characters
                                    return Err(UriParserError::InvalidHostname);
                                }

                                // part after ':' is port
                                if decode {

                                    // port number cannot contain encoded characters
                                    return Err(UriParserError::InvalidPortNumber);
                                }
                                if let Ok(port) = buffer.parse::<u16>() {
                                    return Ok((UriCredentials::None, hostname_or_username, Some(port)));
                                } else {
                                    return Err(UriParserError::InvalidPortNumber);
                                }
                            },
                            Some(_) => return Err(UriParserError::InvalidHostname)
                        }
                    }
                },
                Some(&'@') => {
                    parser.next();

                    // part before '@' must be username since we didn't find ':'
                    let credentials = UriCredentials::Username(if decode {
                        uri_decode(&buffer)
                    } else {
                        buffer
                    });
                    if Some(&'[') == parser.peek() {
                        return parse_ipv6(parser, credentials);
                    }
                    return parse_hostname(parser, credentials);
                },
                Some(&'/') | Some(&'\\') | Some(&'?') | Some(&'#') | None => {

                    // part before '/', '\', '?', '#' must be hostname
                    if decode {

                        // hostname cannot contain encoded characters
                        return Err(UriParserError::InvalidHostname);
                    }
                    return Ok((UriCredentials::None, buffer, None));
                },
                Some(_) => return Err(UriParserError::InvalidHostname)
            }
        }
    }

    fn parse_hostname(mut parser: &mut Peekable<Chars>, credentials: UriCredentials) -> Result<(UriCredentials,String,Option<u16>), UriParserError> {

        // parse hostname
        let mut decode = false;
        let mut buffer = String::new();
        loop {
            match parser.peek() {
                Some(&c) if (c == '%') || (c <= '+') => {
                    parser.next();
                    decode = true;

                    // potentially valid character, keep parsing
                    buffer.push(c);
                },
                Some(&c) if
                    ((c >= 'a') && (c <= 'z'))
                    || ((c >= 'A') && (c <= 'Z'))
                    || ((c >= '0') && (c <= '9'))
                    || ((c >= '$') && (c <= '.'))   // one of: $%&'()*+,-.
                    || (c == '!')
                    || (c == ';')
                    || (c == '=')
                    || (c == '_')
                    || (c == '~')
                    || c.is_numeric()
                    || c.is_alphabetic()
                => {
                    parser.next();

                    // valid character, keep parsing
                    buffer.push(c);
                },
                Some(&':') => {
                    if decode {

                        // hostname cannot contain encoded characters
                        return Err(UriParserError::InvalidHostname);
                    }
                    return parse_portnumber(parser, credentials, buffer);
                },
                Some(&'/') | Some(&'\\') | Some(&'?') | Some(&'#') | None => {

                    // part before '/', '\', '?', '#' must be hostname
                    if decode {

                        // hostname cannot contain encoded characters
                        return Err(UriParserError::InvalidHostname);
                    }
                    return Ok((credentials, buffer, None));
                },
                Some(_) => return Err(UriParserError::InvalidHostname)
            }
        }
    }

    fn parse_ipv6(mut parser: &mut Peekable<Chars>, credentials: UriCredentials) -> Result<(UriCredentials,String,Option<u16>), UriParserError> {

        // IPv6 address must begin with '['
        if Some('[') != parser.next() {
            return Err(UriParserError::InternalError);
        }
        let mut buffer = String::new();
        buffer.push('[');

        // parse IPv6 address
        loop {
            match parser.peek() {
                Some(&c) if c.is_digit(16) || (c == ':') || (c == '.') => {
                    parser.next();

                    // valid character, keep parsing
                    buffer.push(c);
                },
                Some(&']') => {
                    parser.next();
                    buffer.push(']');

                    // check if there is an optional port number
                    match parser.peek() {
                        Some(&':') => {
                            return parse_portnumber(parser, credentials, buffer);
                        },
                        Some(&'/') | Some(&'\\') | Some(&'?') | Some(&'#') | None => {
                            return Ok((credentials, buffer, None));
                        },
                        Some(_) => return Err(UriParserError::InvalidIPv6)
                    }
                },
                _ => return Err(UriParserError::InvalidIPv6)
            }
        }
    }

    fn parse_portnumber(mut parser: &mut Peekable<Chars>, credentials: UriCredentials, hostname: String) -> Result<(UriCredentials,String,Option<u16>), UriParserError> {

        // port number must begin with ':'
        if Some(':') != parser.next() {
            return Err(UriParserError::InternalError);
        }
        let mut buffer = String::new();

        // parse port number
        loop {
            match parser.peek() {
                Some(&c) if (c >= '0') || (c <= '9') => {
                    parser.next();

                    // valid character, keep parsing
                    buffer.push(c);
                },
                Some(&'/') | Some(&'\\') | Some(&'?') | Some(&'#') | None => {
                    if let Ok(port) = buffer.parse::<u16>() {
                        return Ok((credentials, hostname, Some(port)));
                    } else {
                        return Err(UriParserError::InvalidPortNumber);
                    }
                },
                Some(_) => return Err(UriParserError::InvalidPortNumber)
            }
        }
    }

    // check first character; it could tell us if we're parsing an IPv6 address
    if Some(&'[') == parser.peek() {
        return parse_ipv6(parser, UriCredentials::None);
    }
    return parse_hostname_or_userinfo(parser);
}

fn uri_decode(_text: &str) -> String {
    unimplemented!();
}

