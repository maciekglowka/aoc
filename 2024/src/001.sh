left=()
right=()
declare -A count

while read -r line; do
  parts=($line)
  left+=(${parts[0]})
  right+=(${parts[1]})
  count[${parts[1]}]=$(("${count[${parts[1]}]}" + 1))
done < $1

left_sorted=($(printf '%s\n' "${left[@]}"|sort -g))
right_sorted=($(printf '%s\n' "${right[@]}"|sort -g))

total=0

for i in "${!left_sorted[@]}"; do
  d=$((${left_sorted[i]} - ${right_sorted[i]}))
  if (( d < 0 )); then
    d=$((-1 * $d))
  fi
  total=$(($total + $d))
done

echo $total

total=0 

for i in "${!left_sorted[@]}"; do
  d="${count["${left_sorted[i]}"]}"
  if [ -n "$d" ]; then
    total=$(($total + $d * "${left_sorted[i]}"))
  fi
done

echo $total
