SCRIPT="$0"
SCRIPT_DIR="$( cd "$( dirname "${SCRIPT}" )" && pwd )"

cd "$SCRIPT_DIR"/Report || exit
latexmk -pvc -pdf -jobname=Report report.tex
