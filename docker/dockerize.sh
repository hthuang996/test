#!/bin/bash

catagory=$1
name=$2
opt_version=$3
echo $catagory $name $opt_version

if [ -z $catagory ]
then
  echo "Lack parameter catagory"
  exit 1
fi

if [ -z $name ]
then
  echo "Lack parameter name"
  exit 1
fi

IMAGEID="cherima/$catagory-$name"

if [ ! -z $opt_version ]
then
  if [[ $opt_version != "--version" ]]
  then
    echo "The third parameter wrong"
    exit 2
  fi

  PACKAGE_VERSION=$(cat package.json \
  | grep version \
  | head -1 \
  | awk -F: '{ print $2 }' \
  | sed 's/[",]//g' \
  | tr -d '[[:space:]]')
  IMAGEID="$IMAGEID:$PACKAGE_VERSION"
fi

echo "Building $IMAGEID ..."
docker build -t $IMAGEID .
# docker tag $IMAGEID "registry.cn-chengdu.aliyuncs.com/cherima/dante-node:latest"