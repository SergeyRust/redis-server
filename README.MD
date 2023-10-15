                    REDIS-SERVER
 
REST API для 

 - получения(GET) /api/get_data/<key>
 - загрузки(POST) /api/post_data/<key>, content-type=text/plain в теле запроса
 - изменения(PUT) /put_data/<key>, content-type=text/plain в теле запроса
 - удаления(DELETE) /delete_data/<key>

данных, хранящихся в redis. По ключу (path_variable <key>) добавляем/изменяем/получаем/удаяем данные,  
согласно правилам REST API.

        Запуск
docker compose -f local.docker-compose.yaml up

