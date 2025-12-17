for id in $(seq 100 199); do
  echo "Downloading book $id"
  curl -L "https://www.gutenberg.org/cache/epub/$id/pg${id}.txt" \
       -o "books/book_${id}.txt"
done