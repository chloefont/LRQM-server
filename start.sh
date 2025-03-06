#!/bin/sh

# Run the migrate script
sqlx migrate run

# Start the app
./lrqm_server