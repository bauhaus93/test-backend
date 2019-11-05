FROM postgres

EXPOSE 5432

COPY --chown=postgres:postgres init/ /docker-entrypoint-initdb.h/
COPY --chown=postgres:postgres config/ /var/lib/postgresql/initial_config/
