rust生成java开发中的一些常用固定的代码，比如controller，entry，xml等，可以节省一些编写基础代码的时间

第一个rust小项目，写的不好，但是确实是可用的😄，可以生成一些模板代码再后后续手工修改

使用方式，更新config.ini中的数据库配置和包名，然后在项目根目录，执行cargo run，即可生成全部表对应实体的增删改查的基础代码
如果只想生成指定表的代码，只需要运行时加上表名即可(多个的话直接写多个参数即可)，如cargo run admin_user admin_log 
文件会输出到src/output下，输出样例可以参考output_demo


写的并不通用，是根据个人日常开发习惯来写的，有如下限制
1）只支持mysql
2）表只支持有自增主键，否则会panic
3）并非支持所有的mysql字段类型，只支持src/config/db.rs中的几种


个人日常写java的目录结构如下:
src
|____dto                        // 用来service内部传递的数据结构
| |____filter                   // 专门用来向mapper传递过滤条件的层
|____vo                         // 视图层
|____mapper                     // mapper接口层
|____controller                 // 动作接收层
|____service                    // 业务逻辑层
|____domain                     // 实体层
resource    
| |____dao                      // xml文件层



Mapper/xml层写了六个基本函数
getBy`Pk`(一般就是getById了，根据主键查询)
getBy`Pks`(一般就是getByIds了，根据多个主键查询)
getPageList(分页查询列表)
getCount(查询数量，一般配合getPageList使用)
getByFilter(根据过滤条件查询)
add(添加)
update(更新)


controller/service中有四个函数
getPageList(分页查询)
getDetail(查询一个详情)
add(添加)
update(更新)
如果实体字段多一点，那么add和update编写也会比较麻烦，直接生成方便很多