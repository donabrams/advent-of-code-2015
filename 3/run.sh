#!usr/bin/env bash

elm-make --yes main.elm --output raw-main.js > /dev/null
bash ./elm-stuff/packages/laszlopandy/elm-console/1.0.3/elm-io.sh raw-main.js main.js > /dev/null
node main.js
rm raw-main.js
rm main.js