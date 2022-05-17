

#[cfg(test)]
mod test {
    use tdog_core::Cmd;
    use crate::*;

    #[test]
    fn parse_json() {
        {
            let dl = r#"
            {
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": "123"
                            }
                        },
                        "to": {
                            "sqlite": {
                                "file": "abc"
                            }
                        },
                        "options": {
                            "watch": true
                        }
                    }
                }
            }
            "#;

            let _v: Config = serde_json::from_str(dl).expect("Ok");
            // dbg!(&v);


            // No license - allow custom message for license details.
            let dl = r#"
            {
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": "123"
                            }
                        },
                        "to": {
                            "sqlite": {
                                "file": "abc"
                            }
                        },
                        "options": {
                            "watch": true
                        }
                    }
                }
            }
            "#;

            let _v: Config = serde_json::from_str(dl).expect("Ok");
            // dbg!(&v);


            // Mysql with ip and port.
            let dl = r#"
            {
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": "123"
                            }
                        },
                        "to": {
                            "mysql": {
                                "addr": {
                                    "ip": "127.0.0.1",
                                    "port": 3306
                                },
                                "user": "root",
                                "pass": "my-secret-pw",
                                "schema_name": "stripe_acc_x"
                            }
                        },
                        "options": {
                            "watch": true
                        }
                    }
                }
            }
            "#;

            let _v: Config = serde_json::from_str(dl).expect("Ok");
            // dbg!(&v);


            // Mysql with socket.
            let dl = r#"
            {
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": "123"
                            }
                        },
                        "to": {
                            "mysql": {
                                "addr": {
                                    "socket": "/tmp/mysql.sock"
                                },
                                "user": "root",
                                "pass": "my-secret-pw",
                                "schema_name": "stripe_acc_x"
                            }
                        },
                        "options": {
                            "watch": true
                        }
                    }
                }
            }
            "#;

            let _v: Config = serde_json::from_str(dl).expect("Ok");
            // dbg!(&v);
        }
    }


    #[test]
    fn parse_json_set_defaults() {
        {
            // Mysql, no `schema_name` (assert that a default is set).
            // Note: Serde treats null the same as a missing JSON key (results in Option::None).
            let dl = r#"
            {
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": "123"
                            }
                        },
                        "to": {
                            "mysql": {
                                "addr": {
                                    "socket": "/tmp/mysql.sock"
                                },
                                "user": "root",
                                "pass": "my-secret-pw",
                                "db_name": null,
                                "schema_name": null
                            }
                        },
                        "options": {
                            "watch": true
                        }
                    }
                }
            }
            "#;

            let mut v: Config = serde_json::from_str(dl).expect("Ok");
            v.set_defaults();

            let x = || {
                match &v.cmd {
                    Cmd::Download(dl) => {
                        match &dl.to {
                            Engine::SQLite(_) => {}
                            Engine::MySQL(x) => {
                                return x.schema_name.clone();
                            }
                            Engine::Postgres(_) => {}
                        }
                    }
                }
                None
            };
            assert!(x().is_some());


            dbg!(&v);
        }
    }

}