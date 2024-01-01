# test.dia
# Usage: ./format-conversion.sh test
DIR=.
for i in svg pdf png
do 	 
dia --nosplash --export=$DIR/${1}.${i} ${DIR}/${1}.dia
done


