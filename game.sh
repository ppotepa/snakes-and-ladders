#!/bin/bash

# Snakes and Ladders - Single Screen Bash Game
# Roll dice, climb ladders, dodge snakes!

BOARD_SIZE=100
POSITION=1

# Define snakes (position -> move_to)
declare -A SNAKES=([99]=80 [87]=24 [62]=18 [51]=6 [37]=3)

# Define ladders (position -> move_to)
declare -A LADDERS=([3]=22 [5]=14 [9]=31 [20]=38 [28]=84 [51]=67 [72]=91)

roll_dice() {
  echo $((1 + RANDOM % 6))
}

move_player() {
  local roll=$1
  POSITION=$((POSITION + roll))
  
  if [ $POSITION -gt $BOARD_SIZE ]; then
    POSITION=$((BOARD_SIZE - (POSITION - BOARD_SIZE)))
  fi
  
  if [ ${SNAKES[$POSITION]:-0} -ne 0 ]; then
    echo "SNAKE at $POSITION. Sliding down to ${SNAKES[$POSITION]}"
    POSITION=${SNAKES[$POSITION]}
  elif [ ${LADDERS[$POSITION]:-0} -ne 0 ]; then
    echo "LADDER at $POSITION. Climbing up to ${LADDERS[$POSITION]}"
    POSITION=${LADDERS[$POSITION]}
  fi
}

draw_board() {
  clear
  echo "------------------------------------------"
  echo " SNAKES AND LADDERS - PROOF OF CONCEPT"
  echo " LADDERS GIVE, SNAKES TAKE"
  echo "------------------------------------------"
  echo ""
  echo "Position: $POSITION / $BOARD_SIZE"
  echo ""
  
  local progress=$((POSITION * 40 / BOARD_SIZE))
  printf "Progress: ["
  for ((i=0; i<40; i++)); do
    if [ $i -lt $progress ]; then printf "="; else printf " "; fi
  done
  printf "] %d%%\n" $((POSITION * 100 / BOARD_SIZE))
  echo ""
}

main() {
  while true; do
    draw_board
    
    if [ $POSITION -ge $BOARD_SIZE ]; then
      echo "You win. Reached position $POSITION."
      break
    fi
    
    read -p "Press ENTER to roll dice... (q to quit) " input
    [ "$input" = "q" ] && exit 0
    
    roll=$(roll_dice)
    echo "Rolled: $roll"
    move_player $roll
    echo "Now at: $POSITION"
    sleep 1.5
  done
}

main
