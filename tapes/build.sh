# vars
POSITION_BUILD=`pwd`/target/tapes
POSITION_TAPES=`pwd`/tapes

# setup
mkdir -p $POSITION_BUILD/{cache,current,data}
export UTPM_CACHE_DIR=$POSITION_BUILD/cache
export UTPM_DATA_DIR=$POSITION_BUILD/data
export UTPM_CURRENT_DIR=$POSITION_BUILD/current

# populate
utpmc() {
    utpm ws create -mf --name $1 --version $2 --namespace $3 > /dev/null
    utpm ws link -f > /dev/null
    rm -r $UTPM_CURRENT_DIR/*
}

for i in $(seq 1 5); do utpmc test 1.0.$i local; done
for i in $(seq 1 2); do utpmc random $i.0.$i bar; done
utpmc hello 1.2.3 foo

# script
vhs tapes/help.tape
vhs tapes/list.tape
vhs tapes/tree.tape

unset UTPM_CACHE_DIR
unset UTPM_DATA_DIR
unset UTPM_CURRENT_DIR