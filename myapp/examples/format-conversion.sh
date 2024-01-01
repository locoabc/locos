DIR=.
for i in svg pdf png
do 	 
dia --nosplash --export=$DIR/${1}.${i} ${DIR}/${1}
done


