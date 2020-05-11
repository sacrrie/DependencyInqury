# DependencyInqury


This is a short and simple webpage scraper to help identify package dependencies in the Github repositories by RUST while I was doing part-time jobs at a local company.

It's a small micro-service to read information about a company's GitHub repos that are written in RUST and record their interdependencies with a database.

As the folder names show, the codes are distributed as:

dependencies_map: scrapping the dependencies from the GitHub webpage and store them into a database.

dependencies_microservices: Runs the answering services to inquiry call. The system uses a UDP messaging system deployed locally.

dependencies_client: The client to conduct an inquiry.
