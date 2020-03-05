# Publis

*Publish client-side application bundles to S3*


### Environment variables

```bash
export AWS_ACCESS_KEY_ID=""
export AWS_SECRET_ACCESS_KEY=""
export AWS_REGION=""
```

### Commands

Upload bundle to S3 run:
```
    publis --bucket <bucket> --root <root>
```

Add a website policy to the bucket run:
```
    publis --bucket <bucket> --add-policy
```

