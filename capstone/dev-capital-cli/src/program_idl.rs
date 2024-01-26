use solana_idlgen::idlgen;

idlgen!({
  "version": "0.1.0",
  "name": "dev_capital",
  "instructions": [
    {
      "name": "initDevFund",
      "accounts": [
        {
          "name": "funder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "dev",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "devFund",
          "isMut": true,
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
          "name": "lamports",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initDevConfig",
      "accounts": [
        {
          "name": "dev",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "devFund",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "devConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployOffsets",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployData",
          "isMut": true,
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
          "name": "ot6Len",
          "type": "u32"
        },
        {
          "name": "ot5Len",
          "type": "u32"
        },
        {
          "name": "origLen",
          "type": "u32"
        }
      ]
    },
    {
      "name": "accountSizeOffsets",
      "accounts": [
        {
          "name": "dev",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "devFund",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "devConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployOffsets",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "accountSizeData",
      "accounts": [
        {
          "name": "dev",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "devFund",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "devConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployData",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deployOffsets",
      "accounts": [
        {
          "name": "dev",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "devFund",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "devConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployOffsets",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deployData",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "DeployData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "DeployOffsets",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "DevConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "ot6Len",
            "type": "u32"
          },
          {
            "name": "ot6Index",
            "type": "u32"
          },
          {
            "name": "ot5Len",
            "type": "u32"
          },
          {
            "name": "ot5Index",
            "type": "u32"
          },
          {
            "name": "dataOrigLen",
            "type": "u32"
          },
          {
            "name": "dev",
            "type": "publicKey"
          },
          {
            "name": "devFund",
            "type": "publicKey"
          },
          {
            "name": "deployOffsets",
            "type": "publicKey"
          },
          {
            "name": "deployData",
            "type": "publicKey"
          },
          {
            "name": "devConfigBump",
            "type": "u8"
          },
          {
            "name": "deployOffsetsBump",
            "type": "u8"
          },
          {
            "name": "deployDataBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "DevFund",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "funder",
            "type": "publicKey"
          },
          {
            "name": "dev",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
  ,
  "metadata": {
    "address": "5MHA6ForrBUbPbom2x231cNsMCQvE4VCpQ7F6rKMt8bS"
  }


}
);
