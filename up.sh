#! /bin/bash
mongorestore --uri="mongodb://root:nKuvxmOUuP9ti528@dev2-platform.cluster-cti10lnrh4rb.us-west-2.docdb.amazonaws.com:27017/dev2-good-form-template?authSource=admin&readPreference=primary" --sslCAFile="cert.pem"--nsInclude="dev2-good-form-template.trial-2707ce73-793a-4734-9060-08ad41f68ad5" ./dump

