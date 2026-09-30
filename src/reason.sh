#!/bin/bash
for filename in cache/unreasoned/*.owl; do
      echo "Reasoning $(basename "$filename" .owl).owl"
      # robot reason -i "$filename" --reasoner ELK -o "cache/reasoned/$(basename "$filename" .owl).owl"
      if [ -f "cache/reasoned/$(basename "$filename" .owl).owl"]; then
          echo "Successfully reasoned $(basename "$filename" .owl).owl"
done


# 48874
