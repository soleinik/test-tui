#!/bin/bash
#


TICKERS=('^SPX' NVDA FBTC WULF MARA HIMS IREN HUT)




for ticker in ${TICKERS[@]}; do
	for i in {1..20}; do
		if [[ ${#i} < 2 ]]; then
			filename=2024-11-0${i}-${ticker}.json.not
		else
			filename=2024-11-${i}-${ticker}.json.not
		fi	       
	touch $filename
	done
done

