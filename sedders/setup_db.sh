###########################################
# !/usr/bin/env bash                      #
# Author Mauricio Jourdan                 #
# Inicializa la base de datos lumau.db    #
###########################################

# Read Vars from .env file
source .env

cd $(dirname "$0")

# Variables
DATETIME=$(date +%Y-%m-%d\ %H:%M:%S)
DATETIME_LOG=$(date +%Y-%m-%d_%H-%M-%S)
DIR_LOG="log"
LOGFILE="$DIR_LOG/init_db-$DATETIME_LOG.log"
SQL_DIR="sql"

echo -e '--- Inicio proceso setup_db.sh ---\n' > $LOGFILE

# Create Tables
echo -e '* Creando tablas' >> $LOGFILE
echo -e '    - Tabla: admin_user\n' >> $LOGFILE

# sqlite3 ../$DATABASE < $SQL_DIR/db.sql 

# Create admin user
echo -e '* Creando usuario admin\n' >> $LOGFILE

sqlite3 "../$DATABASE_URL" \
".param set :name '$ADMIN_NAME'" \
".param set :url '$ADMIN_URL'" \
".param set :front_deploy '$ADMIN_FRONT_DEPLOY'" \
".param set :email '$ADMIN_EMAIL'" \
".param set :password '$ADMIN_PASSWORD'" \
".param set :status '$ADMIN_STATUS'" \
".param set :role '$ADMIN_ROLE'" \
".param set :updated '$DATETIME'" \
".read $SQL_DIR/create_admin_user.sql"

echo -e '--- Fin proceso setup_db.sh ---' >> $LOGFILE


