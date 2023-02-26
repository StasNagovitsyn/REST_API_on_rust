Запрос POST на добавление автора с заданным именен:
curl -X POST -H "Content-Type: application/json" -d "{ ""author_name"": ""timofei"" }" http://localhost:3000/api/v1/author
Ответ:
[{"authors_id":15,"name":"timofei"}]

Запрос GET на вывод всех авторов:
curl http://localhost:3000/api/v1/authors
Ответ:
[{"authors_id":1,"name":"Aleksandr Pushkin"},{"authors_id":2,"name":"Uri Lermontov"},{"authors_id":3,"name":"Aleksandr Duma"},{"authors_id":4,"name":"Lev Tolstoi"},{"authors_id":5,"name":"Aleksandr Blok"},{"authors_id":6,"name":"Vladimir Moykovski"},{"authors_id":7,"name":"Anna Ahmatova"},{"authors_id":8,"name":"value"},{"authors_id":9,"name":"value"},{"authors_id":10,"name":"Lena"},{"authors_id":11,"name":"value"}]

Запрос GET получить автора по id:
curl http://localhost:3000/api/v1/author/10
Ответ:
{"authors_id":10,"name":"Lena"}

Запрос GET поиск автора по имени:
curl http://localhost:3000/api/v1/author/search?author_name=stas
Ответ:
{"authors_id":12,"name":"stas"}

Запрос PUT изменить автора по id:
curl -X PUT -H "Content-Type: application/json" -d "{ ""author_name"": ""Bertucho"" }" http://localhost:3000/api/v1/author/10
Ответ:
{"author_name":"Bertucho"}

Запрос DEL на удаление автора  по id:
curl -i -X DELETE http://localhost:3000/api/v1/author/10
Ответ:
{"msg":"Author Deleted"}



