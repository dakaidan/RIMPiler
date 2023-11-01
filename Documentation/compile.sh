# A script to help compile and live preview the reports using latexmk.
# takes three optional arguments:
# -b: only compile the BSPR
# -r: only compile the report
# -c: to clean the auxiliary files
# If no arguments are given, both the BSPR and the report are compiled.
SCRIPT="$0"
SCRIPT_DIR="$( cd "$( dirname "${SCRIPT}" )" && pwd )"

COMPILE_BSPR=true
COMPILE_REPORT=true

while getopts ":brc" opt; do
  case $opt in
    b)
      COMPILE_REPORT=false
      ;;
    r)
      COMPILE_BSPR=false
      ;;
    c)
      echo "Cleaning the auxiliary files"
      cd "$SCRIPT_DIR"/BSPR || exit
      latexmk -c -jobname=BSPR main.tex
      cd "$SCRIPT_DIR"/Report || exit
      latexmk -c -jobname=Report main.tex
      exit
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      ;;
  esac
done

if [ "$COMPILE_BSPR" = true ] && [ "$COMPILE_REPORT" = true ]; then
  echo "Compiling the BSPR and the report"
  "$SCRIPT_DIR"/BSPR.sh &
  BSPR_PID=$!
  "$SCRIPT_DIR"/Report.sh &
  REPORT_PID=$!

  wait $BSPR_PID
  wait $REPORT_PID
  exit
fi

if [ "$COMPILE_BSPR" = true ]; then
  echo "Compiling the BSPR"
  "$SCRIPT_DIR"/BSPR.sh
  exit
fi

if [ "$COMPILE_REPORT" = true ]; then
  echo "Compiling the report"
  "$SCRIPT_DIR"/Report.sh
  exit
fi