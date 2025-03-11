# lrqm_server

## Installation

Install Rustup
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Copy the `.env.vars` file to `.env` and fill in the values.
```bash
cp .env.var .env
```

Install the cargo dependencies
```bash
cargo build
```

Create the database if it doesn't exist
```bash
sqlx database create
```

Run the migrations
```bash
sqlx migrate run
```

## Running

Run the docker-compose file
```bash
docker-compose up
```

Run the server
```bash
cargo run
```

## Api documentation

```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "lrqm_server",
    "description": "",
    "license": {
      "name": ""
    },
    "version": "0.1.0"
  },
  "paths": {
    "/events": {
      "post": {
        "tags": [
          "api_events"
        ],
        "description": "Create an event",
        "operationId": "event_create",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/NewEvent"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Event created"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/events/": {
      "get": {
        "tags": [
          "api_events"
        ],
        "description": "Get all events",
        "operationId": "events_list",
        "responses": {
          "200": {
            "description": "Events found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/events/:event_id": {
      "get": {
        "tags": [
          "api_events"
        ],
        "description": "Get an event by id",
        "operationId": "get_event",
        "parameters": [
          {
            "name": "event_id",
            "in": "path",
            "description": "The event id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Event found"
          },
          "404": {
            "description": "Event not found"
          }
        }
      }
    },
    "/events/:event_id/active_users": {
      "get": {
        "tags": [
          "api_events"
        ],
        "description": "Get the number of active users of an event",
        "operationId": "get_event_active_users_number",
        "parameters": [
          {
            "name": "event_id",
            "in": "path",
            "description": "The event id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Active users number found"
          },
          "404": {
            "description": "Event not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/events/:event_id/meters": {
      "get": {
        "tags": [
          "api_events"
        ],
        "description": "Get the total meters of an event",
        "operationId": "event_total_meters",
        "parameters": [
          {
            "name": "event_id",
            "in": "path",
            "description": "The event id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Total meters found"
          },
          "404": {
            "description": "Event not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/measures/:measure_id": {
      "put": {
        "tags": [
          "api_measures"
        ],
        "description": "Edit meters",
        "operationId": "edit_meters",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/EditMeters"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Meters edited"
          },
          "400": {
            "description": "Bad request"
          },
          "404": {
            "description": "Not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/measures/:measure_id/stop": {
      "put": {
        "tags": [
          "api_measures"
        ],
        "description": "Stop measuring",
        "operationId": "stop_meters",
        "responses": {
          "200": {
            "description": "Measure stopped"
          },
          "400": {
            "description": "Bad request"
          },
          "404": {
            "description": "Not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/measures/start": {
      "post": {
        "tags": [
          "api_measures"
        ],
        "description": "Start measuring",
        "operationId": "start_measuring",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/NewMeasure"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Measure started"
          },
          "400": {
            "description": "Bad request"
          },
          "404": {
            "description": "Not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/users": {
      "post": {
        "tags": [
          "api_users"
        ],
        "description": "Create a user",
        "operationId": "user_create",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/NewUser"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "User created"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/users/": {
      "get": {
        "tags": [
          "api_users"
        ],
        "description": "Get all users",
        "operationId": "users_list",
        "responses": {
          "200": {
            "description": "Users found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/users/:user_id": {
      "get": {
        "tags": [
          "api_users"
        ],
        "description": "Get a user by id",
        "operationId": "get_user",
        "parameters": [
          {
            "name": "user_id",
            "in": "path",
            "description": "The user id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "User found"
          },
          "404": {
            "description": "User not found"
          }
        }
      },
      "patch": {
        "tags": [
          "api_users"
        ],
        "description": "Edit a user",
        "operationId": "patch_user",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/PatchUser"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "User edited"
          },
          "404": {
            "description": "User not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/users/:user_id/meters": {
      "get": {
        "tags": [
          "api_users"
        ],
        "description": "Get the total contribution of a user",
        "operationId": "get_user_total_meters",
        "parameters": [
          {
            "name": "user_id",
            "in": "path",
            "description": "The user id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Total meters found"
          },
          "404": {
            "description": "User not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/users/:user_id/time": {
      "get": {
        "tags": [
          "api_users"
        ],
        "description": "Get the total time spent by a user",
        "operationId": "get_user_total_time_spent",
        "parameters": [
          {
            "name": "user_id",
            "in": "path",
            "description": "The user id",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Total time found"
          },
          "404": {
            "description": "User not found"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    }
  },
  "components": {
    "schemas": {
      "EditMeters": {
        "type": "object",
        "required": [
          "meters"
        ],
        "properties": {
          "meters": {
            "type": "integer",
            "format": "int32"
          }
        }
      },
      "NewEvent": {
        "type": "object",
        "required": [
          "name",
          "start_date",
          "end_date",
          "meters_goal"
        ],
        "properties": {
          "end_date": {
            "type": "string",
            "format": "date-time"
          },
          "meters_goal": {
            "type": "integer",
            "format": "int32"
          },
          "name": {
            "type": "string"
          },
          "start_date": {
            "type": "string",
            "format": "date-time"
          }
        }
      },
      "NewMeasure": {
        "type": "object",
        "required": [
          "user_id"
        ],
        "properties": {
          "contributors_number": {
            "type": [
              "integer",
              "null"
            ],
            "format": "int32"
          },
          "user_id": {
            "type": "integer",
            "format": "int32"
          }
        }
      },
      "NewUser": {
        "type": "object",
        "required": [
          "username",
          "bib_id",
          "event_id"
        ],
        "properties": {
          "bib_id": {
            "type": "string"
          },
          "event_id": {
            "type": "integer",
            "format": "int32"
          },
          "username": {
            "type": "string"
          }
        }
      },
      "PatchUser": {
        "type": "object",
        "properties": {
          "bib_id": {
            "type": [
              "string",
              "null"
            ]
          },
          "event_id": {
            "type": [
              "integer",
              "null"
            ],
            "format": "int32"
          },
          "total_meters": {
            "type": [
              "integer",
              "null"
            ],
            "format": "int32"
          },
          "username": {
            "type": [
              "string",
              "null"
            ]
          }
        }
      }
    }
  }
}
```