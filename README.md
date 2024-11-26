# connections
the connections

## development

- go into `oauth2-proxy.cfg` and change the `redirect_url` to the localhost one
- run `docker-compose up`

## environment variables
- `./oauth2-proxy.env`
  - `OAUTH2_PROXY_COOKIE_SECRET`
  - `OAUTH2_PROXY_CLIENT_SECRET`
- `./mail-handler.env`
  - `IMAP_ADDR`
  - `IMAP_PORT`
  - `IMAP_USERNAME`
  - `NON_ANONYMOUS_ADDRESS`
  - `IMAP_PASSWORD`
  - `IMAP_MAILBOX_NAME`
  - `ADMIN_EMAILS`: comma-separated list of email addresses
  - `LISTMONK_TOKEN`: format `api_user:token`
  - `LISTMONK_LIST_ID`: ID of list in Listmonk to subscribe users to