# Now that our image has been initialized, we can clean up our APT caches.
apt-get clean
rm -rf /var/lib/apt/lists
