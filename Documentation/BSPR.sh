SCRIPT="$0"
SCRIPT_DIR="$( cd "$( dirname "${SCRIPT}" )" && pwd )"

cd "$SCRIPT_DIR"/BSPR || exit
latexmk -pvc -pdf -jobname=BSPR main.tex