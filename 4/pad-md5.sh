#!usr/bin/env bash

PASSWORD=$(cat password.txt)

while [[ ${MD5:0:5} != "00000" ]]; do
	NUM=$[$NUM + 1]
	MD5=$(printf "%s%d" ${PASSWORD} ${NUM} | md5)
#	echo ${NUM} ${MD5} ${MD5:0:5}
done

echo ${NUM}
