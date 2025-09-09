DROP DATABASE IF EXISTS sky_take_out;
CREATE DATABASE sky_take_out;

DROP TABLE IF EXISTS address_book;
CREATE TABLE address_book (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    user_id bigint NOT NULL  ,
    consignee varchar(50) DEFAULT NULL  ,
    sex varchar(2) DEFAULT NULL  ,
    phone varchar(11) NOT NULL  ,
    province_code varchar(12)    DEFAULT NULL  ,
    province_name varchar(32)    DEFAULT NULL  ,
    city_code varchar(12)    DEFAULT NULL  ,
    city_name varchar(32)    DEFAULT NULL  ,
    district_code varchar(12)    DEFAULT NULL  ,
    district_name varchar(32)    DEFAULT NULL  ,
    detail varchar(200)    DEFAULT NULL  ,
    label varchar(100)    DEFAULT NULL  ,
    is_default smallint NOT NULL  DEFAULT 0  ,
    PRIMARY KEY (id)
);

ALTER SEQUENCE address_book_id_seq RESTART WITH 2;

DROP TABLE IF EXISTS category;
CREATE TABLE category (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    type int DEFAULT NULL  ,
    name varchar(32) NOT NULL  ,
    sort int NOT NULL  DEFAULT 0  ,
    status int DEFAULT NULL  ,
    create_time timestamp(0) DEFAULT NULL  ,
    update_time timestamp(0) DEFAULT NULL  ,
    create_user bigint DEFAULT NULL  ,
    update_user bigint DEFAULT NULL  ,
    PRIMARY KEY (id),
    CONSTRAINT idx_category_name UNIQUE (name)
);

ALTER SEQUENCE category_id_seq RESTART WITH 23;

INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (11,1,'酒水饮料',10,1,'2022-06-09 22:09:18','2022-06-09 22:09:18',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (12,1,'传统主食',9,1,'2022-06-09 22:09:32','2022-06-09 22:18:53',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (13,2,'人气套餐',12,1,'2022-06-09 22:11:38','2022-06-10 11:04:40',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (15,2,'商务套餐',13,1,'2022-06-09 22:14:10','2022-06-10 11:04:48',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (16,1,'蜀味烤鱼',4,1,'2022-06-09 22:15:37','2022-08-31 14:27:25',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (17,1,'蜀味牛蛙',5,1,'2022-06-09 22:16:14','2022-08-31 14:39:44',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (18,1,'特色蒸菜',6,1,'2022-06-09 22:17:42','2022-06-09 22:17:42',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (19,1,'新鲜时蔬',7,1,'2022-06-09 22:18:12','2022-06-09 22:18:28',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (20,1,'水煮鱼',8,1,'2022-06-09 22:22:29','2022-06-09 22:23:45',1,1);
INSERT INTO category OVERRIDING SYSTEM VALUE VALUES (21,1,'汤类',11,1,'2022-06-10 10:51:47','2022-06-10 10:51:47',1,1);

DROP TABLE IF EXISTS dish;
CREATE TABLE dish (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    name varchar(32) NOT NULL  ,
    category_id bigint NOT NULL  ,
    price decimal(10,2) DEFAULT NULL  ,
    image varchar(255) DEFAULT NULL  ,
    description varchar(255) DEFAULT NULL  ,
    status int  DEFAULT 1  ,
    create_time timestamp(0) DEFAULT NULL  ,
    update_time timestamp(0) DEFAULT NULL  ,
    create_user bigint DEFAULT NULL  ,
    update_user bigint DEFAULT NULL  ,
    PRIMARY KEY (id),
    CONSTRAINT idx_dish_name UNIQUE (name)
);

ALTER SEQUENCE dish_id_seq RESTART WITH 70;

INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (46,'王老吉',11,6.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/41bfcacf-7ad4-4927-8b26-df366553a94c.png','',1,'2022-06-09 22:40:47','2022-06-09 22:40:47',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (47,'北冰洋',11,4.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/4451d4be-89a2-4939-9c69-3a87151cb979.png','还是小时候的味道',1,'2022-06-10 09:18:49','2022-06-10 09:18:49',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (48,'雪花啤酒',11,4.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/bf8cbfc1-04d2-40e8-9826-061ee41ab87c.png','',1,'2022-06-10 09:22:54','2022-06-10 09:22:54',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (49,'米饭',12,2.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/76752350-2121-44d2-b477-10791c23a8ec.png','精选五常大米',1,'2022-06-10 09:30:17','2022-06-10 09:30:17',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (50,'馒头',12,1.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/475cc599-8661-4899-8f9e-121dd8ef7d02.png','优质面粉',1,'2022-06-10 09:34:28','2022-06-10 09:34:28',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (51,'老坛酸菜鱼',20,56.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/4a9cefba-6a74-467e-9fde-6e687ea725d7.png','原料：汤，草鱼，酸菜',1,'2022-06-10 09:40:51','2022-06-10 09:40:51',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (52,'经典酸菜鮰鱼',20,66.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/5260ff39-986c-4a97-8850-2ec8c7583efc.png','原料：酸菜，江团，鮰鱼',1,'2022-06-10 09:46:02','2022-06-10 09:46:02',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (53,'蜀味水煮草鱼',20,38.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/a6953d5a-4c18-4b30-9319-4926ee77261f.png','原料：草鱼，汤',1,'2022-06-10 09:48:37','2022-06-10 09:48:37',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (54,'清炒小油菜',19,18.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/3613d38e-5614-41c2-90ed-ff175bf50716.png','原料：小油菜',1,'2022-06-10 09:51:46','2022-06-10 09:51:46',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (55,'蒜蓉娃娃菜',19,18.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/4879ed66-3860-4b28-ba14-306ac025fdec.png','原料：蒜，娃娃菜',1,'2022-06-10 09:53:37','2022-06-10 09:53:37',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (56,'清炒西兰花',19,18.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/e9ec4ba4-4b22-4fc8-9be0-4946e6aeb937.png','原料：西兰花',1,'2022-06-10 09:55:44','2022-06-10 09:55:44',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (57,'炝炒圆白菜',19,18.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/22f59feb-0d44-430e-a6cd-6a49f27453ca.png','原料：圆白菜',1,'2022-06-10 09:58:35','2022-06-10 09:58:35',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (58,'清蒸鲈鱼',18,98.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/c18b5c67-3b71-466c-a75a-e63c6449f21c.png','原料：鲈鱼',1,'2022-06-10 10:12:28','2022-06-10 10:12:28',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (59,'东坡肘子',18,138.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/a80a4b8c-c93e-4f43-ac8a-856b0d5cc451.png','原料：猪肘棒',1,'2022-06-10 10:24:03','2022-06-10 10:24:03',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (60,'梅菜扣肉',18,58.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/6080b118-e30a-4577-aab4-45042e3f88be.png','原料：猪肉，梅菜',1,'2022-06-10 10:26:03','2022-06-10 10:26:03',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (61,'剁椒鱼头',18,66.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/13da832f-ef2c-484d-8370-5934a1045a06.png','原料：鲢鱼，剁椒',1,'2022-06-10 10:28:54','2022-06-10 10:28:54',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (62,'金汤酸菜牛蛙',17,88.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/7694a5d8-7938-4e9d-8b9e-2075983a2e38.png','原料：鲜活牛蛙，酸菜',1,'2022-06-10 10:33:05','2022-06-10 10:33:05',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (63,'香锅牛蛙',17,88.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/f5ac8455-4793-450c-97ba-173795c34626.png','配料：鲜活牛蛙，莲藕，青笋',1,'2022-06-10 10:35:40','2022-06-10 10:35:40',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (64,'馋嘴牛蛙',17,88.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/7a55b845-1f2b-41fa-9486-76d187ee9ee1.png','配料：鲜活牛蛙，丝瓜，黄豆芽',1,'2022-06-10 10:37:52','2022-06-10 10:37:52',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (65,'草鱼2斤',16,68.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/b544d3ba-a1ae-4d20-a860-81cb5dec9e03.png','原料：草鱼，黄豆芽，莲藕',1,'2022-06-10 10:41:08','2022-06-10 10:41:08',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (66,'江团鱼2斤',16,119.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/a101a1e9-8f8b-47b2-afa4-1abd47ea0a87.png','配料：江团鱼，黄豆芽，莲藕',1,'2022-06-10 10:42:42','2022-06-10 10:42:42',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (67,'鮰鱼2斤',16,72.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/8cfcc576-4b66-4a09-ac68-ad5b273c2590.png','原料：鮰鱼，黄豆芽，莲藕',1,'2022-06-10 10:43:56','2022-06-10 10:43:56',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (68,'鸡蛋汤',21,4.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/c09a0ee8-9d19-428d-81b9-746221824113.png','配料：鸡蛋，紫菜',1,'2022-06-10 10:54:25','2022-06-10 10:54:25',1,1);
INSERT INTO dish OVERRIDING SYSTEM VALUE VALUES (69,'平菇豆腐汤',21,6.00,'https://sky-itcast.oss-cn-beijing.aliyuncs.com/16d0a3d6-2253-4cfc-9b49-bf7bd9eb2ad2.png','配料：豆腐，平菇',1,'2022-06-10 10:55:02','2022-06-10 10:55:02',1,1);

DROP TABLE IF EXISTS dish_flavor;
CREATE TABLE dish_flavor (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    dish_id bigint NOT NULL  ,
    name varchar(32) DEFAULT NULL  ,
    value varchar(255) DEFAULT NULL  ,
    PRIMARY KEY (id)
);

ALTER SEQUENCE dish_flavor_id_seq RESTART WITH 104;

INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (40,10,'甜味','["无糖","少糖","半糖","多糖","全糖"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (41,7,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (42,7,'温度','["热饮","常温","去冰","少冰","多冰"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (45,6,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (46,6,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (47,5,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (48,5,'甜味','["无糖","少糖","半糖","多糖","全糖"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (49,2,'甜味','["无糖","少糖","半糖","多糖","全糖"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (50,4,'甜味','["无糖","少糖","半糖","多糖","全糖"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (51,3,'甜味','["无糖","少糖","半糖","多糖","全糖"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (52,3,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (86,52,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (87,52,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (88,51,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (89,51,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (92,53,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (93,53,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (94,54,'忌口','["不要葱","不要蒜","不要香菜"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (95,56,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (96,57,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (97,60,'忌口','["不要葱","不要蒜","不要香菜","不要辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (101,66,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (102,67,'辣度','["不辣","微辣","中辣","重辣"]');
INSERT INTO dish_flavor OVERRIDING SYSTEM VALUE VALUES (103,65,'辣度','["不辣","微辣","中辣","重辣"]');

DROP TABLE IF EXISTS employee;
CREATE TABLE employee (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    name varchar(32) NOT NULL  ,
    username varchar(32) NOT NULL  ,
    password varchar(64) NOT NULL  ,
    phone varchar(11) NOT NULL  ,
    sex varchar(2) NOT NULL  ,
    id_number varchar(18) NOT NULL  ,
    status int NOT NULL  DEFAULT 1  ,
    create_time timestamp(0) DEFAULT NULL  ,
    update_time timestamp(0) DEFAULT NULL  ,
    create_user bigint DEFAULT NULL  ,
    update_user bigint DEFAULT NULL  ,
    PRIMARY KEY (id),
    CONSTRAINT idx_username UNIQUE (username)
);

ALTER SEQUENCE employee_id_seq RESTART WITH 2;

INSERT INTO employee OVERRIDING SYSTEM VALUE VALUES (1,'管理员','admin','123456','13812312312','1','110101199001010047',1,'2022-02-15 15:51:20','2022-02-17 09:16:20',10,1);

DROP TABLE IF EXISTS order_detail;
CREATE TABLE order_detail (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    name varchar(32) DEFAULT NULL  ,
    image varchar(255) DEFAULT NULL  ,
    order_id bigint NOT NULL  ,
    dish_id bigint DEFAULT NULL  ,
    setmeal_id bigint DEFAULT NULL  ,
    dish_flavor varchar(50) DEFAULT NULL  ,
    number int NOT NULL  DEFAULT 1  ,
    amount decimal(10,2) NOT NULL  ,
    PRIMARY KEY (id)
);

ALTER SEQUENCE order_detail_id_seq RESTART WITH 5;

DROP TABLE IF EXISTS orders;
CREATE TABLE orders (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    number varchar(50) DEFAULT NULL  ,
    status int NOT NULL  DEFAULT 1  ,
    user_id bigint NOT NULL  ,
    address_book_id bigint NOT NULL  ,
    order_time timestamp(0) NOT NULL  ,
    checkout_time timestamp(0) DEFAULT NULL  ,
    pay_method int NOT NULL  DEFAULT 1  ,
    pay_status smallint NOT NULL  DEFAULT 0  ,
    amount decimal(10,2) NOT NULL  ,
    remark varchar(100) DEFAULT NULL  ,
    phone varchar(11) DEFAULT NULL  ,
    address varchar(255) DEFAULT NULL  ,
    user_name varchar(32) DEFAULT NULL  ,
    consignee varchar(32) DEFAULT NULL  ,
    cancel_reason varchar(255) DEFAULT NULL  ,
    rejection_reason varchar(255) DEFAULT NULL  ,
    cancel_time timestamp(0) DEFAULT NULL  ,
    estimated_delivery_time timestamp(0) DEFAULT NULL  ,
    delivery_status smallint NOT NULL  DEFAULT 1  ,
    delivery_time timestamp(0) DEFAULT NULL  ,
    pack_amount int DEFAULT NULL  ,
    tableware_number int DEFAULT NULL  ,
    tableware_status smallint NOT NULL  DEFAULT 1  ,
    PRIMARY KEY (id)
)   ;

ALTER SEQUENCE orders_id_seq RESTART WITH 4;

DROP TABLE IF EXISTS setmeal;
CREATE TABLE setmeal (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    category_id bigint NOT NULL  ,
    name varchar(32) NOT NULL  ,
    price decimal(10,2) NOT NULL  ,
    status int  DEFAULT 1  ,
    description varchar(255) DEFAULT NULL  ,
    image varchar(255) DEFAULT NULL  ,
    create_time timestamp(0) DEFAULT NULL  ,
    update_time timestamp(0) DEFAULT NULL  ,
    create_user bigint DEFAULT NULL  ,
    update_user bigint DEFAULT NULL  ,
    PRIMARY KEY (id),
    CONSTRAINT idx_setmeal_name UNIQUE (name)
);

ALTER SEQUENCE setmeal_id_seq RESTART WITH 32;

DROP TABLE IF EXISTS setmeal_dish;
CREATE TABLE setmeal_dish (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    setmeal_id bigint DEFAULT NULL  ,
    dish_id bigint DEFAULT NULL  ,
    name varchar(32) DEFAULT NULL  ,
    price decimal(10,2) DEFAULT NULL  ,
    copies int DEFAULT NULL  ,
    PRIMARY KEY (id)
);

ALTER SEQUENCE setmeal_dish_id_seq RESTART WITH 47;

DROP TABLE IF EXISTS shopping_cart;
CREATE TABLE shopping_cart (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    name varchar(32) DEFAULT NULL  ,
    image varchar(255) DEFAULT NULL  ,
    user_id bigint NOT NULL  ,
    dish_id bigint DEFAULT NULL  ,
    setmeal_id bigint DEFAULT NULL  ,
    dish_flavor varchar(50) DEFAULT NULL  ,
    number int NOT NULL  DEFAULT 1  ,
    amount decimal(10,2) NOT NULL  ,
    create_time timestamp(0) DEFAULT NULL  ,
    PRIMARY KEY (id)
);

ALTER SEQUENCE shopping_cart_id_seq RESTART WITH 9;

DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY  ,
    openid varchar(45) DEFAULT NULL  ,
    name varchar(32) DEFAULT NULL  ,
    phone varchar(11) DEFAULT NULL  ,
    sex varchar(2) DEFAULT NULL  ,
    id_number varchar(18) DEFAULT NULL  ,
    avatar varchar(500) DEFAULT NULL  ,
    create_time timestamp(0) DEFAULT NULL,
    PRIMARY KEY (id)
);

ALTER SEQUENCE "user_id_seq" RESTART WITH 4;