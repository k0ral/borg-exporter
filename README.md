# Prometheus exporter for [Borg](https://github.com/borgbackup/borg)

[![Built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

## Exported metrics

All metrics are gauges:
- `borg_archives_total`: number of available archives
- `borg_archives_latest_time`: timestamp of the latest archive
- `borg_archives_size`: original size of all archives
- `borg_archives_deduplicated_size`: deduplicated size of all archives
- `borg_archives_compressed_size`: compressed size of all archives
- `borg_archives_deduplicated_compressed_size`: compressed, deduplicated size of all archives
- `borg_chunks_total`: number of chunks
- `borg_chunks_deduplicated_total`: number of unique chunks

## How-to

### Build docker image

```bash
nix build
```

### Load docker image

```bash
docker load < ./result
```

### Start docker container

Here are the main parameters to customize the behavior of the exporter:
1. Optional: if the borg repository needs a remote SSH access, you will need to mount a working SSH configuration into `/tmp/.ssh`
2. Optional: if the borg repository is encrypted, the [`BORG_PASSPHRASE` environment variable](https://borgbackup.readthedocs.io/en/stable/faq.html?highlight=BORG_PASSPHRASE#how-can-i-specify-the-encryption-passphrase-programmatically) must be set to the encryption passphrase
3. Mandatory: set the URL of the borg repository to monitor

#### As a standalone container

```bash
# Fill the following variables first
SSH_FOLDER=~/.ssh  # (1)
BORG_PASSPHRASE="fill_me"  # (2)
BORG_REPOSITORY="fill_me"  # (3)
PORT=9884

docker run -it --rm -p "${PORT}:${PORT} -v "$SSH_FOLDER:/tmp/.ssh:ro" -e "BORG_PASSPHRASE=${BORG_PASSPHRASE}" k0ral/borg-exporter --repository "${BORG_REPOSITORY}"
```

#### As part of a docker-compose application

In `docker-compose.yml`:
```yaml
borg-exporter:
  image: k0ral/borg-exporter
  volumes:
    - ~/.ssh:/tmp/.ssh:ro  # (1)
  environment:
    - BORG_PASSPHRASE=fill_me  # (2)
  command:
    - borg-exporter
    - --repository
    - fill_me  # (3)
  expose:
    - 9884
  restart: always
```

### Register exporter in prometheus

In `prometheus.yml`:
```yaml
scrape_configs:
  - job_name: 'borg-exporter'
    scrape_interval: 1h
    scrape_timeout: 5m
    static_configs:
      - targets: ['borg-exporter:9884']
```
