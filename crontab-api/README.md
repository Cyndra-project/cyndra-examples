# crontab-api

A service that calls URLs at specified cron intervals.

The service exposes a `/set-schedule` endpoint that accepts a schedule and a URL as form data and persists the cron job with `cyndra_persist` between runs.

```
curl -v http://localhost:8000/set-schedule\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule='*/2 * * * * *'&url='example.com'"
```

The example demonstrates implementation of a custom service with [`cyndra_runtime::Service`](https://docs.cyndra.rs/examples/custom-service), usage of [`cyndra_persist`](https://docs.cyndra.rs/resources/cyndra-persist), and how to run an `axum::Server` and a number of cron job processes in parallel.
