# Prerequisites
- setup JAVA_HOME
- setup ANDROID_HOME
- use android studio setup target physical device
  - check usb cable, adb
- ```npx react-native init platform-pda```

# Dev
- ```npm start``` // start metro in one terminal
- ```npm run android``` // build, install & debug in another terminal

# Build
```cd android && ./gradlew bundleRelease && ./gradlew assembleRelease```

# Icons 
- 72*72 ic_launcher.png in mipmap-hdpi
- 48*48 ic_launcher.png in mipmap-mdpi
- 96*96 ic_launcher.png in mipmap-xhdpi.
- 144*144 ic_launcher.png in mipmap-xxhdpi.
- 192*192 ic_launcher.png in mipmap-xxxhdpi