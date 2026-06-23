package com.rustoon.app

import dev.dioxus.main.WryActivity
import android.webkit.WebView
import android.webkit.WebSettings

class MainActivity : WryActivity() {
    override fun onWebViewCreate(webView: WebView) {
        super.onWebViewCreate(webView)
        
        val settings = webView.settings
        
        // Enable JavaScript (already on by default, but be explicit)
        settings.javaScriptEnabled = true
        
        // Enable DOM storage — required for localStorage (reading history, bookmarks)
        settings.domStorageEnabled = true
        
        // Enable database storage
        settings.databaseEnabled = true
        
        // Allow file access for bundled assets
        settings.allowFileAccess = true
        settings.allowContentAccess = true
        
        // Enable responsive viewport
        settings.useWideViewPort = true
        settings.loadWithOverviewMode = true
        
        // Set a proper cache mode
        settings.cacheMode = WebSettings.LOAD_DEFAULT
        
        // Enable mixed content (HTTPS page loading HTTP resources)
        settings.mixedContentMode = WebSettings.MIXED_CONTENT_NEVER_ALLOW
    }
}
