export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:crates/ctp-futures/v_current
#./target/release/examples/ctp_query -c configs/ctp_quote.json
while ((true)); do

	cur_date="$(date "+%Y-%m-%d %H:%M:%S")"
	cur_hour="$(date +%H)"
	cur_min="$(date +%M)"

	if [ $cur_hour = 15 -a $cur_min = 32 ]; then
		echo $(date '+%Y%m%d %H:%M:%S')'  start ctp_query save pdv'
		./target/release/examples/ctp_query -c configs/ctp_quote.json
		sleep 60
	fi

	sleep 1
done
