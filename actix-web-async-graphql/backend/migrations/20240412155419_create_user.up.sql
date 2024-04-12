-- Add up migration script here
-- Add up migration script here
create table `user`(
                       `id` int(0) unsigned  auto_increment not null,
                       `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci not null,
                       `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
                       `cred` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL,
                       PRIMARY KEY (`id`) USING BTREE
)ENGINE = InnoDB AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO `user` VALUES (1, 'ok@budshome.com', '我谁24ok32', '5ff82b2c0076cc8b00e5cddb');
INSERT INTO `user` VALUES (2, 'oka@budshome.com', '我s谁24ok32', '5ff83f4b00e8fda000e5cddc');
INSERT INTO `user` VALUES (3, 'oka2@budshome.com', '我2s谁24ok32', '5ffd710400b6b84e000349f8');

SET FOREIGN_KEY_CHECKS = 1;