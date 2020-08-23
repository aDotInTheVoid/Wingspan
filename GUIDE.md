# Development Guide
To recreate the test images, please run

```bash
WINGSPAN_UPDATE_IMAGES=1 cargo test
```

or on powershell
```powershell
$env:WINGSPAN_UPDATE_IMAGES=1; cargo test;  Remove-Item Env:\WINGSPAN_UPDATE_IMAGES
```