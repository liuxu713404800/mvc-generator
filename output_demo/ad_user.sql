CREATE TABLE `ad_user` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `account` varchar(20) NOT NULL COMMENT '账号',
  `password` varchar(50) NOT NULL COMMENT '密码',
  `name` varchar(50) NOT NULL COMMENT '用户名',
  `avatar` varchar(255) DEFAULT NULL COMMENT '用户头像',
  `status` tinyint(3) unsigned NOT NULL DEFAULT '1' COMMENT '用户状态，0：禁用，1，可用',
  `is_admin` tinyint(3) unsigned NOT NULL COMMENT '是否为管理员',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  `update_time` datetime NOT NULL ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1419863967240687618 DEFAULT CHARSET=utf8mb4;