CREATE TABLE config_meta (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	app_id TEXT NOT NULL,
	code TEXT NOT NULL,
	property TEXT NOT NULL,
	column_name TEXT NOT NULL,
	description TEXT NOT NULL,
	sort INTEGER NOT NULL,
	gmt_create INTEGER NOT NULL,
	gmt_modified INTEGER NOT NULL
);

INSERT INTO config_meta (id,app_id,code,property,column_name,description,sort,gmt_create,gmt_modified) VALUES 
(1,'platform','UserType','code','varchar1','编码',10,1577836800,1577836800)
,(2,'platform','UserType','name','varchar2','名字',20,1577836800,1577836800)
,(3,'platform','UserType','description','varchar3','说明',30,1577836800,1577836800)
,(4,'platform','UserType','img','varchar4','图标',40,1577836800,1577836800)
;
