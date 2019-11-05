FROM postgres

COPY --chown=postgres:postgres init/ /docker-entrypoint-initdb.d/
COPY --chown=postgres:postgres config/ /var/lib/postgresql/initial_config/
