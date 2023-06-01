## Useful sources:

- https://codereview.stackexchange.com/questions/153710/sql-database-for-a-social-network
- https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example

## Project Structure

- api
  - actual web server that handles requests
- application
  - all the business logic that acessses the database
- domain
  - actual database models
- infrastructure
  - db migrations
- shared
  - data types shared across all crates
