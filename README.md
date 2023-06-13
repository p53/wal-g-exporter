# Wal-g exporter

Prometheus exporter for exporting wal-g backup tool data about backups and additional data related to backup process.

```bash
wal-g-exporter --help
{"time":"2023-06-13T12:54:24.667420148+02:00","message":"Starting application","module_path":"wal_g_exporter","file":"src/lib.rs","line":20,"level":"INFO","target":"wal_g_exporter","thread":"main","thread_id":140640611135168,"mdc":{}}
Exports wal-g data in prometheus format

Usage: wal-g-exporter [OPTIONS] --target <TARGET> --db-user <DB_USER> --db-password <DB_PASSWORD> --db-name <DB_NAME> --db-data-dir <DB_DATA_DIR> --aws-region <AWS_REGION> --aws-s3-force-path-style <AWS_S3_FORCE_PATH_STYLE> --aws-endpoint <AWS_ENDPOINT> --aws-access-key-id <AWS_ACCESS_KEY_ID> --aws-secret-access-key <AWS_SECRET_ACCESS_KEY> --walg-s3-ca-cert-file <WALG_S3_CA_CERT_FILE> --walg-s3-prefix <WALG_S3_PREFIX>

Options:
  -t, --target <TARGET>                                    [possible values: postgres]
      --db-host <DB_HOST>                                  [env: DB_HOST=] [default: localhost]
      --db-port <DB_PORT>                                  [env: DB_PORT=] [default: 5432]
      --db-user <DB_USER>                                  [env: DB_USER=]
      --db-password <DB_PASSWORD>                          [env: DB_PASSWORD=]
      --db-name <DB_NAME>                                  [env: DB_NAME=]
      --db-data-dir <DB_DATA_DIR>                          [env: DB_DATA_DIR=]
      --aws-region <AWS_REGION>                            [env: AWS_REGION=]
      --aws-s3-force-path-style <AWS_S3_FORCE_PATH_STYLE>  [env: AWS_S3_FORCE_PATH_STYLE=]
      --aws-endpoint <AWS_ENDPOINT>                        [env: AWS_ENDPOINT=]
      --aws-access-key-id <AWS_ACCESS_KEY_ID>              [env: AWS_ACCESS_KEY_ID=]
      --aws-secret-access-key <AWS_SECRET_ACCESS_KEY>      [env: AWS_SECRET_ACCESS_KEY=]
      --walg-s3-ca-cert-file <WALG_S3_CA_CERT_FILE>        [env: WALG_S3_CA_CERT_FILE=]
      --walg-s3-prefix <WALG_S3_PREFIX>                    [env: WALG_S3_PREFIX=]
      --collection-interval <COLLECTION_INTERVAL>          [env: COLLECTION_INTERVAL=] [default: 30]
      --port <PORT>                                        [env: PORT=] [default: 8080]
  -h, --help                                               Print help
  -V, --version                                            Print version
```

Example environment:

```bash
source > /tmp/env <<EOF
export DB_PASSWORD="feFjWkY9hFrIcsedtKh85FPRlfFkYcpNT7sRUwjwM1mA290aJL31nlSy8D4l6czV"
export DB_USER="postgres"
export DB_NAME="foo"
export WALG_S3_CA_CERT_FILE="/home/test/.mc/certs/CAs/example.pem"
export AWS_ENDPOINT="minio.apps.my.com"
export AWS_ACCESS_KEY_ID="my-access_id"
export AWS_SECRET_ACCESS_KEY="2Mv7KxUvircNYmZ9fobCnuaRkfxWFybL"
export WALG_S3_PREFIX="s3://bucket-name/path/to/your/wal-g/backups"
export AWS_REGION="us-east-2"
export AWS_S3_FORCE_PATH_STYLE="true"
export DB_DATA_DIR="/var/lib"
EOF
```

Now you can run:

```bash
wal-g-exporter --target=postgres
```

Fetch prometheus metrics:

```bash
curl http://localhost:8080
```