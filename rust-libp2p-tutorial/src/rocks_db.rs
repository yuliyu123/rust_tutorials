use rocksdb::{Error, Options, WriteBatch, DB};

#[test]
fn test_open_put_and_get() {
    let path = "_path_for_rocksdb_storage";
    {
        let db = DB::open_default(path).unwrap();
        assert!(db.put(b"k1", b"v1111").is_ok());
        let r: Result<Option<Vec<u8>>, Error> = db.get(b"k1");

        assert_eq!(r.unwrap().unwrap(), b"v1111");
        assert!(db.delete(b"k1").is_ok());
        assert!(db.get(b"k1").unwrap().is_none());
    }
    let _ = DB::destroy(&Options::default(), path);
}

#[test]
fn test_write_batch_clear() {
    let mut batch = WriteBatch::default();
    batch.put(b"1", b"1");
    batch.put(b"2", b"2");
    batch.put(b"3", b"3");
    batch.put(b"4", b"4");
    assert_eq!(batch.len(), 4);
    batch.clear();
    assert_eq!(batch.len(), 0);
    assert!(batch.is_empty());
}
