# generate key without running an agent (less secure as less entropy)
gpg2 --gen-key --pinentry-mode loopback
# or
gpg --gen-key
# note that --allow-freeform-uid allows chars < 5

# list secret keys
gpg --list-secret-keys
gpg --list-keys

# export public key
gpg --export -a "nom" > public.key

# export priv key
gpg --export-secret-key -a "nom" > private.key
gpg --export-secret-key -a "nom" | pbcopy

# decrypt a message
gpg --decrypt text.gpg
