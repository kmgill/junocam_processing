#!/bin/bash

if [ "x$JUNOBASE"  == "x" ]; then
    echo Error: \$JUNOBASE not specified. Please do so before running.
    exit 1
fi

which curl > /dev/null
if [ $? -eq 1 ]; then 
    echo Error: 'curl' not found. Please install before continuing.
    exit 1
fi

which wget > /dev/null
if [ $? -eq 1 ]; then 
    echo Error: 'wget' not found. Please install before continuing.
    exit 1
fi


function mkdir_if_not_exist() {
    if [ ! -d $1 ]; then
        mkdir -p $1
    fi
}


mkdir_if_not_exist $JUNOBASE
cd $JUNOBASE

mkdir_if_not_exist kernels
cd kernels

mkdir_if_not_exist download

pushd download
wget --mirror --no-parent -N -X /pub/naif/JUNO/misc/ https://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/ck
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/pck/pck00010.tpc
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/fk/juno_v12.tf
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/ik/juno_junocam_v03.ti
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/lsk/naif0012.tls
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/sclk/JNO_SCLKSCET.00138.tsc
curl -O ftp://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/spk/juno_struct_v04.bsp
popd

mkdir_if_not_exist ck 
cp download/naif.jpl.nasa.gov/pub/naif/JUNO/kernels/ck/juno_sc_rec* ck/
#mv download/naif.jpl.nasa.gov/pub/naif/JUNO/kernels/ck/juno_sc_raw* ck/

mkdir_if_not_exist pck
cp download/pck00010.tpc pck/

mkdir_if_not_exist fk
cp download/juno_v12.tf fk/

mkdir_if_not_exist ik
cp download/juno_junocam_v03.ti ik/

mkdir_if_not_exist lsk
cp download/naif0012.tls lsk/

mkdir_if_not_exist sclk 
cp download/JNO_SCLKSCET.00138.tsc sclk/jno_sclkscet_00074.tsc

mkdir_if_not_exist spk
cp download/juno_struct_v04.bsp spk/