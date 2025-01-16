# Tasker app

Rust tokio task experiments

Run async tasks in web app. 
Support add, enumerate and stop async tasks

## tasker operations

### list tasks
curl -X GET http://localhost:8080/task

### add task
curl -X POST  --data '{"name":"test"}' http://localhost:8080/task

### stop task
curl -X PUT  http://localhost:8080/task/stop/{task_id}


