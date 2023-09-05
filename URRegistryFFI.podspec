Pod::Spec.new do |spec|
  spec.name         = "URRegistryFFI"
  spec.version      = "0.2.3"
  spec.summary      = "An BC-UR registry implementation with rust-lang"
  spec.homepage     = "https://github.com/KeystoneHQ/keystone-sdk-rust"
  spec.license      = { :type => 'Copyright', :text => 'Copyright 2023 Keystone' }
  spec.author       = "Keystone"
  spec.social_media_url   = "https://twitter.com/KeystoneWallet"
  spec.swift_version = "5.6"
  spec.platform = :ios, '13.0'
  spec.static_framework = true
  spec.source       = { :http => "https://github.com/KeystoneHQ/keystone-sdk-rust/releases/download/sdk-0.1.4/URRegistryFFI.xcframework.zip", :type => "zip" }
  spec.ios.vendored_frameworks = 'URRegistryFFI.xcframework'
end
