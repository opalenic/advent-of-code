#!/bin/bash

gnuplot <(echo 'plot "house_presents.data" with dots, "max_house_presents.data" with lines; pause -1 "Press any key to continue..."')
