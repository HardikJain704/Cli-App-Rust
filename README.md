A rust Cli App which stores data i.e id , name , and address which creates a json file and stores data inside it , and we can then fetch data from it , and delete it using subcommands.

 # Running

 1. Add command to first create a entry and then can automatically creates a exampl.json file

 cargo run add -i id -n "name" -a "address"

# example 

``` cargo run add -i 1 -n "Hardik" -a "Bangalore" ```

2. fetch command to fetch the details to contacts we have added 

 ``` cargo run fetch --id id ```
 
 # example 
 ``` cargo run fetch --id 1 ```

3. Delete command to delete the contact details

``` cargo run delete -m id ```
 
# example 
``` cargo run delete -m 1 ```
