export type Ctf = {
  "version": "0.1.0",
  "name": "week1",
  "instructions": [
    {
      "name": "initializeCtfProfile",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeChallenge1",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "vault1"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "completeChallenge2",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "vault2"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "completeChallenge3",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK idgaf if we get exploited"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "auth"
              }
            ]
          }
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeChallenge4",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "vault4"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "metadata_program"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ],
            "programId": {
              "kind": "account",
              "type": "publicKey",
              "path": "metadata_program"
            }
          }
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeChallenge5",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK idgaf if we get exploited"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "auth"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "completeCtf",
      "accounts": [
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deleteProfile",
      "accounts": [
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "profile",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "profile"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "path": "owner"
              }
            ]
          },
          "relations": [
            "owner"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Profile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "score",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "Completed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BumpError",
      "msg": "Invalid bump"
    },
    {
      "code": 6001,
      "name": "AlreadyCompleted",
      "msg": "You Already Completed this Challenge"
    },
    {
      "code": 6002,
      "name": "AmountError",
      "msg": "That's not the right amount. Try again"
    },
    {
      "code": 6003,
      "name": "InvalidOwner",
      "msg": "Invalid owner"
    },
    {
      "code": 6004,
      "name": "InvalidPrecision",
      "msg": "Invalid number of decimals! Try again"
    },
    {
      "code": 6005,
      "name": "WrongName",
      "msg": "That's not the right name for the token. Try again"
    },
    {
      "code": 6006,
      "name": "Challenge3Authority",
      "msg": "const [auth_pda, _bump] = PublicKey.findProgramAddressSync(Buffer.from(auth"
    },
    {
      "code": 6007,
      "name": "MissingChallenge1",
      "msg": "You're missing challenge 1 completion. Go do it!"
    },
    {
      "code": 6008,
      "name": "MissingChallenge2",
      "msg": "You're missing challenge 2 completion. Go do it!"
    },
    {
      "code": 6009,
      "name": "MissingChallenge3",
      "msg": "You're missing challenge 3 completion. Go do it!"
    },
    {
      "code": 6010,
      "name": "MissingChallenge4",
      "msg": "You're missing challenge 4 completion. Go do it!"
    },
    {
      "code": 6011,
      "name": "MissingChallenge5",
      "msg": "You're missing challenge 5 completion. Go do it!"
    }
  ]
}

  export const IDL: Ctf = {
    "version": "0.1.0",
    "name": "week1",
    "instructions": [
      {
        "name": "initializeCtfProfile",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            }
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      },
      {
        "name": "completeChallenge1",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "ata",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "vault",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "vault1"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "account": "Mint",
                  "path": "mint"
                }
              ]
            }
          },
          {
            "name": "mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      },
      {
        "name": "completeChallenge2",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "ata",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "vault",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "vault2"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "account": "Mint",
                  "path": "mint"
                }
              ]
            }
          },
          {
            "name": "mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      },
      {
        "name": "completeChallenge3",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "ata",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "authority",
            "isMut": true,
            "isSigner": false,
            "docs": [
              "CHECK idgaf if we get exploited"
            ],
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "auth"
                }
              ]
            }
          },
          {
            "name": "mint",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      },
      {
        "name": "completeChallenge4",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "ata",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "vault",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "vault4"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "account": "Mint",
                  "path": "mint"
                }
              ]
            }
          },
          {
            "name": "metadata",
            "isMut": false,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "metadata"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "metadata_program"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "account": "Mint",
                  "path": "mint"
                }
              ],
              "programId": {
                "kind": "account",
                "type": "publicKey",
                "path": "metadata_program"
              }
            }
          },
          {
            "name": "mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "metadataProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      },
      {
        "name": "completeChallenge5",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "ata",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "authority",
            "isMut": true,
            "isSigner": false,
            "docs": [
              "CHECK idgaf if we get exploited"
            ],
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "auth"
                }
              ]
            }
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      },
      {
        "name": "completeCtf",
        "accounts": [
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            }
          },
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      },
      {
        "name": "deleteProfile",
        "accounts": [
          {
            "name": "owner",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "profile",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "string",
                  "value": "profile"
                },
                {
                  "kind": "account",
                  "type": "publicKey",
                  "path": "owner"
                }
              ]
            },
            "relations": [
              "owner"
            ]
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": []
      }
    ],
    "accounts": [
      {
        "name": "Profile",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "owner",
              "type": "publicKey"
            },
            {
              "name": "score",
              "type": "u8"
            },
            {
              "name": "bump",
              "type": "u8"
            }
          ]
        }
      },
      {
        "name": "Completed",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "owner",
              "type": "publicKey"
            }
          ]
        }
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "BumpError",
        "msg": "Invalid bump"
      },
      {
        "code": 6001,
        "name": "AlreadyCompleted",
        "msg": "You Already Completed this Challenge"
      },
      {
        "code": 6002,
        "name": "AmountError",
        "msg": "That's not the right amount. Try again"
      },
      {
        "code": 6003,
        "name": "InvalidOwner",
        "msg": "Invalid owner"
      },
      {
        "code": 6004,
        "name": "InvalidPrecision",
        "msg": "Invalid number of decimals! Try again"
      },
      {
        "code": 6005,
        "name": "WrongName",
        "msg": "That's not the right name for the token. Try again"
      },
      {
        "code": 6006,
        "name": "Challenge3Authority",
        "msg": "const [auth_pda, _bump] = PublicKey.findProgramAddressSync(Buffer.from(auth"
      },
      {
        "code": 6007,
        "name": "MissingChallenge1",
        "msg": "You're missing challenge 1 completion. Go do it!"
      },
      {
        "code": 6008,
        "name": "MissingChallenge2",
        "msg": "You're missing challenge 2 completion. Go do it!"
      },
      {
        "code": 6009,
        "name": "MissingChallenge3",
        "msg": "You're missing challenge 3 completion. Go do it!"
      },
      {
        "code": 6010,
        "name": "MissingChallenge4",
        "msg": "You're missing challenge 4 completion. Go do it!"
      },
      {
        "code": 6011,
        "name": "MissingChallenge5",
        "msg": "You're missing challenge 5 completion. Go do it!"
      }
    ]
  }