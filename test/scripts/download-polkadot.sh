#!/bin/bash

# Exit on any error
set -e

# Grab Polkadot version
branch=$(egrep -o '/polkadot.*#([^\"]*)' $(dirname $0)/../../Cargo.lock | head -1 | sed 's/.*release-//#')
polkadot_release=$(echo $branch | sed 's/#.*//' | sed 's/\/polkadot?branch=tanssi-polkadot-v//')

# Always run the commands from the "test" dir
cd $(dirname $0)/..

if [[ -f tmp/polkadot ]]; then
	POLKADOT_VERSION=$(tmp/polkadot --version)
	if [[ $POLKADOT_VERSION == *$polkadot_release* ]]; then
		exit 0
	else
		echo "Updating polkadot binary..."
		pnpm moonwall download polkadot $polkadot_release tmp
		chmod +x tmp/polkadot
	fi
else
	echo "Polkadot binary not found, downloading..."
	pnpm moonwall download polkadot $polkadot_release tmp
	chmod +x tmp/polkadot
fi
