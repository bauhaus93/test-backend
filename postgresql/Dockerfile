FROM postgres

EXPOSE 5432

COPY --chown=postgres:postgres init/ /docker-entrypoint-initdb.d/
COPY --chown=postgres:postgres config/ /var/lib/postgresql/initial_config/
USER postgres
