# Maint

A CLI tool for managing customer maintenance contracts with point-based tracking.

## Database

Uses SQLite at `~/.maint.db` by default. Override with `MAINT_DB` environment variable.

## Usage

```sh
# Customers
maint add customer --name "Example Corp"
maint list customer
maint show customer 1

# Contracts
maint add contract 1 --start-date 2023-01-01 --end-date 2023-12-31 --total-points 100

# Requests
maint add request 1 --description "Fix production server"

# Work logs
maint add work 1 --worker "John Doe" --description "Troubleshot network issue" --points-used 5

# Show usage from contract start to date
maint usage 1 --date 2023-06-24
```
