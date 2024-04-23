UPDATE testing.counter
    SET count = count + 1
where id = 1
RETURNING $table_fields;