SELECT REGEXP_SUBSTR('abc def ghi', '[a-z]+', 1, 2);
SELECT REGEXP_SUBSTR('abc def GHI', '[a-z]+', 1, 3, 'c');
SELECT REGEXP_SUBSTR('Customers - (NY)','\\([[:alnum:]\-]+\\)');
SELECT REGEXP_SUBSTR('周周周周', '.*', 2);
SELECT REGEXP_SUBSTR('🍣🍣b', 'b', 2);
SELECT REGEXP_SUBSTR('µå周çб周周', '周+', 3, 2);
SELECT REGEXP_SUBSTR('周 周周 周周周 周周周周', '周+', 2, 3);
SELECT REGEXP_SUBSTR('周 周周', '周+', 5);
SELECT REGEXP_SUBSTR(NULL, '');
SELECT REGEXP_SUBSTR('abc def ghi', NULL);
SELECT REGEXP_SUBSTR('abc def ghi', '[a-z]+', NULL);
SELECT REGEXP_SUBSTR('abc def ghi', '[a-z]+', 1, NULL);
SELECT REGEXP_SUBSTR('abc def ghi', '[a-z]+', 1, 2, NULL);
SELECT '======';
DROP TABLE IF EXISTS t1;
CREATE TABLE t1(s String NULL, pat String NULL, pos Int64 NULL, occu Int64 NULL, mt String NULL) Engine = Memory;
INSERT INTO t1 (s, pat, pos, occu, mt) VALUES (NULL, '[a-z]+', 1, 1, ''), ('abc def ghi', NULL, 1, 1, 'c'), ('abc def ghi', '[a-z]+', NULL, 1, 'c'), ('abc def ghi', '[a-z]+', 1, NULL, 'c'), ('abc def ghi', '[a-z]+', 1, 1, NULL), ('abc def ghi', '[a-z]+', 1, 1, 'c');
SELECT s FROM t1 WHERE REGEXP_SUBSTR(s, pat, pos, occu, mt) = 'abc';
